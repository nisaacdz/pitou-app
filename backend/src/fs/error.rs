use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum RenameError {
    /// If the filealready exists and the create file wasn't set to override original file in filename collisions
    NameAlreadyExists,
    /// Includes invalid names or path names
    InvalidPath,
    /// Includes lack of permission to visit or to create a newfile at the given location
    NoPermission,
    /// Occurs when the file is currently opened or in use when the rename is invoked
    FileInUse
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DeleteError {
    /// Occurs when the file is currently opened or in use when the rename is invoked
    FileInUse,
    /// Includes lack of permission to visit or to create a newfile at the given location
    NoPermission,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum CreateFileError {
    /// If the filealready exists and the create file wasn't set to override original file in filename collisions
    NameAlreadyExists,
    /// Includes invalid names or path names
    InvalidPath,
    /// Includes lack of permission to visit or to create a newfile at the given location
    NoPermission,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum GetMetadataError {
    /// If there path doesn't exist or is invalid
    InvalidPath,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum GetPropertiesError {
    /// If there path doesn't exist or is invalid
    InvalidPath,
}
