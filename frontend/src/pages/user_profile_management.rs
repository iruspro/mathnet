use sauron::prelude::*;
pub use crate::messages::Msg;
pub use crate::structs::user::*;
pub use crate::structs::user;
pub use crate::app::App;


pub fn user_profile_management_display(current_state_of_app : &App) -> Node<Msg> {
    let mut user = user::new();
    let mut user_profile_changes = UserChangingProfileData::new();
    node! {
                <div class="container-fluid">
                    <div class="row">
                    <div class="col-4">
                    </div>
                        <div class="col-4">
                            <h1 class="text-center">"User profile"</h1>

<div class="my_card_design">
<div class="card border-dark mb-3" >
<div class="card text-center">
  <div class="card-body">
    <h5 class="card-title">"Edit user profile"</h5>
    <p class="card-text">"There you can view and edit your profile data."</p>
    <form>
    <div class="mb-3">
    <label for="exampleFormControlInput1" class="form-label">"User name"</label>
    <input type="text" 
    class="form-control" 
    aria-describedby="passwordHelpBlock" 
    id="exampleFormControlInput1" 
    placeholder= user.user_name.clone()
    on_input=|input|{Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::ChangeUserName(input.value()))}></input>
  </div>
  <div id="passwordHelpBlock" class="form-text">
        "Enter new user name."
    </div>

    <div class="mb-3">
    <label for="exampleFormControlInput1" class="form-label">"Email address"</label>
    <input type="email" 
    class="form-control" 
    aria-describedby="passwordHelpBlock" 
    id="exampleFormControlInput1" 
    placeholder= user.user_email.clone() 
    on_input=|input| {Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::ChangeUserEmail(input.value()))}></input>
  </div>
  <div id="passwordHelpBlock" class="form-text">
        "Enter new email address."
    </div>

    <div class="mb-3">
    <label for="exampleFormControlInput1" class="form-label">"New password"</label>
    <input type="password" 
    class="form-control" 
    aria-describedby="passwordHelpBlock" 
    id="exampleFormControlInput1" 
    placeholder=user.user_password.clone()
    on_input=|input|{Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::ChangeUserPassword(input.value()))}></input>
  </div>
  <div id="passwordHelpBlock" class="form-text">
        "Enter new password."
    </div>

    <div class="mb-3">
    <label for="exampleFormControlInput1" class="form-label">"New password"</label>
    <input type="password" 
    class="form-control" 
    aria-describedby="passwordHelpBlock" 
    id="exampleFormControlInput1" 
    placeholder=user.user_password.clone()
    on_input=|input|{Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::ChangeUserPasswordConfirmation(input.value()))}></input>
  </div>
  <div id="passwordHelpBlock" class="form-text">
        "Enter new  password again. This step is necessary in order to prevent typos."
    </div>

    <div class="d-grid gap-2 col-6 mx-auto">
    <button class="btn btn-primary" type="button" on_click=|_|{Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::ConfirmChanges)}>"Confirm changes"</button>
  </div>
  <div id="passwordHelpBlock" class="form-text">
         "A confirmation letter will be sent to that address."
    </div>
    </form>
  
  
  </div>


  <div class="my_card_design">
  <div class="card border-dark mb-3" >
  <div class="card text-center">
    <div class="card-body">
      <h5 class="card-title">"Delete account"</h5>
      <form>
      <div class="d-grid gap-2 col-6 mx-auto">
      <button class="btn btn-danger" type="button" on_click=|_|{Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::DeleteAccount)}>"Delete account"</button>
    </div>
    <div id="passwordHelpBlock" class="form-text">
           "After deletion your data will be unrecovable."
      </div>
      </form>
    
    
    </div>
</div>
</div>
</div>
</div>
</div>
</div>
 
<p>
"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras luctus consectetur placerat. Donec non pretium sapien. Donec ac placerat ex. Aenean tempus massa nulla, nec ullamcorper leo tempus eget. Vestibulum at lectus ut libero ullamcorper consectetur. Vestibulum auctor urna venenatis libero eleifend, in sodales odio dictum. Curabitur auctor, massa eget ultricies efficitur, justo nulla porta purus, in laoreet quam turpis ut risus. Aliquam nunc nibh, placerat eget bibendum varius, porta eget tellus. Phasellus in lacinia augue, sed consequat ex. Curabitur laoreet mi nec eros tristique, ac tristique mauris tempus.
Cras felis tellus, egestas ut varius quis, tempor et felis. Mauris in purus auctor, dictum sapien vitae, tristique dolor. Ut felis nisi, feugiat a efficitur ut, imperdiet sit amet tellus. Quisque ante leo, efficitur quis mi hendrerit, posuere congue ante. "
                            </p>
                        </div>
                        <div class="col-4">
                        </div>
                    </div>
                </div>
    }
}