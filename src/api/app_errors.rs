use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "Invalid Id found")]
    InvalidIdError,

    #[display(fmt = "Data not found")]
    DataNotFoundError,

    #[display(fmt= "User not found")]
    UserNotFoundError
}
#[derive(Debug, Display)]    
pub enum AppMessage {
    #[display(fmt = "Data has been inserted!")]
    InsertSuccessMsg,

    #[display(fmt = "Data has been fetched successfully")]
    FetchSuccessMsg,

    #[display(fmt = "Data has been updated successfully")]
    UpdateSuccessMsg,

    #[display(fmt = "Data has been deleted successfully")]
    DeleteSuccessMsg,

    #[display(fmt = "Failed to update the data")]
    UpdateFailedMsg,

    #[display(fmt = "Failed to delete the data")]
    DeleteFailedMsg
}
