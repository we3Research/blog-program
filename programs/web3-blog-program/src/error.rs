use anchor_lang::error_code;

#[error_code]
pub enum BlogError {
    #[msg("Unauthorized: Only the creator can modify this blog.")]
    Unauthorized,
    #[msg("Blog not found.")]
    NotFound,
    #[msg("Author List not init blog.")]
    AuthorListNotInit,
    #[msg("Index Wrong.")]
    IndexWrong,
}