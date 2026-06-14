

pub enum AppError{
    IOError(IOError)
}

pub enum IOError{
    PathError(PathError),
    ReadError(ReadError)

}

pub enum PathError{
    InvalidPath(String),        //path not endend by minecraft or not containded config or mods
    PathNotAbsolute(String),    //
    PathNotExist(String),       //this path isnt exist
    PathMissmatch(String)       //realized by validate, for usual its error by programm, not by user
}

pub enum ReadError{
    FileCantRead(String),
}





impl From<IOError> for AppError{
    fn from(value: IOError) -> Self{
        AppError::IOError(value)
    }
}

impl From<PathError> for IOError{
    fn from(value: PathError) -> Self {
        IOError::PathError(value)
    }
}

impl From<ReadError> for IOError{
    fn from(value: ReadError) -> Self {
        IOError::ReadError(value)
    }
}

impl AppError{
    pub fn code(&self)->u16{
        match self {

            //parsers errors code 11xx
            AppError::IOError(IOError::PathError(PathError::InvalidPath(_))) => 1101,
            AppError::IOError(IOError::PathError(PathError::PathNotAbsolute(_))) => 1102,
            AppError::IOError(IOError::PathError(PathError::PathNotExist(_))) => 1103,
            AppError::IOError(IOError::PathError(PathError::PathMissmatch(_))) => 1104,

            //read errors code 12xx
            AppError::IOError(IOError::ReadError(ReadError::FileCantRead(_))) => 1201,

        }
    }
}

impl AppError{
    pub fn message(&self) -> String{
        match self {

            //parsers errors code 11xx
            AppError::IOError(IOError::PathError(PathError::InvalidPath(s))) => 
                format!("invalid path: {}", s),
            AppError::IOError(IOError::PathError(PathError::PathNotAbsolute(s))) => 
                format!("path not absolute: {}", s),
            AppError::IOError(IOError::PathError(PathError::PathNotExist(s))) => 
                format!("path not exist: {}", s),
            AppError::IOError(IOError::PathError(PathError::PathMissmatch(s))) =>
                format!("path missmatch by ext or type: target is {}",s),


            //read errors code 12xx
            AppError::IOError(IOError::ReadError(ReadError::FileCantRead(s))) =>
                format!("file cant be read: {}", s)

        }
    }
}