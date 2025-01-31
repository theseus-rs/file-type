use crate::format::FileFormat;

pub(crate) const LINGUIST_761352333: FileFormat = FileFormat {
    id: 761_352_333,
    puid: "linguist/761352333",
    name: "Fortran Free Form",
    extensions: &["f03", "f08", "f90", "f95"],
    media_types: &["text/x-fortran"],
    internal_signatures: &[],
    related_formats: &[],
};
