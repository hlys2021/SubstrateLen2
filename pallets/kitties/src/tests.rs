use crate::{Error, mock::*};
use frame_support::{assert_ok,assert_noop};

#[test]
fn it_works_for_create(){
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(),kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), none);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
	}	
}

#[test]
fn it_works_for_breed() {
	let kitty_id = 0;
	let account_id = 1;

	assert_noop!{
		KittiesModule::it_works_for_breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id)
		Error::<Test>::SameKittyId
	}

	assert_noop!{
		KittiesModule::it_works_for_breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1)
		Error::<Test>::InvalidKittyId
	}

	assert_ok!(KittiesModule:create(RuntimeOrigin::signed(account_id)));
	assert_ok!(KittiesModule:create(RuntimeOrigin::signed(account_id)));

	assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

	assert_ok!(KittiesModule::breed(
		RuntimeOrigin::signed(account_id), 
		kitty_id, 
		kitty_id + 1
	))

	let breed_kitty_id = 2;
	assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
	assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
	assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
	assert_eq!(
		KittiesModule::kitty_parents(breed_kitty_id),
		Some((kitty_id, kitty_id + 1))
	);
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(KittiesModule::transfer(
			RuntimeOrigin::signed(recipient),
			kitty_id,
			account_id			
		), Error::<Test>::NotOwner);

		assert_ok!(KittiesModule::transfer(
			RuntimeOrigin::signed(account_id),
			kitty_id,
			recipient			
		));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

		assert_ok!(KittiesModule::transfer(
			RuntimeOrigin::signed(recipient),
			kitty_id, 
			account_id			
		));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

	})

}


#[cfg(test)]
mod tests {
	
	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	fn run_to_block(n: u64) {
		while System::block_number() < n {
			KittiesModule::on_finalize(System::block_number());
			System::set_block_number(System::block_number() + 1);
			KittiesModule::on_initialize(System::block_number());
		}
	}

	#[test]
	fn create_kitty_works() {
		ExtBuilder::default().build().execute_with(|| {
			run_to_block(1);

			assert_ok!(KittiesModule::create(Origin::signed(1)));
			let event = Event::kitty_created(1, 0, Kitty([0; 16]));
			assert_eq!(last_event(), event);
		});
	}

	#[test]
	fn breed_kitty_works() {
		ExtBuilder::default().build().execute_with(|| {
			run_to_block(1);

			assert_ok!(KittiesModule::create(Origin::signed(1)));
			assert_ok!(KittiesModule::create(Origin::signed(2)));
			assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
			let event = Event::kitty_bred(1, 2, 2, Kitty([0; 16]));
			assert_eq!(last_event(), event);
		});
	}

	#[test]
	fn transfer_kitty_works() {
		ExtBuilder::default().build().execute_with(|| {
			run_to_block(1);

			assert_ok!(KittiesModule::create(Origin::signed(1)));
			assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 0));
			let event = Event::kitty_transferred(1, 2, 0);
			assert_eq!(last_event(), event);
		});
	}

	fn last_event() -> Event {
		System::events().into_iter().map(|r| r.event).last().unwrap()
	}
}