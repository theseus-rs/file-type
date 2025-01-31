use crate::format::FileFormat;

pub(crate) const LINGUIST_233: FileFormat = FileFormat {
    id: 233,
    puid: "linguist/233",
    name: "Modelica",
    extensions: &["mo"],
    media_types: &["text/x-modelica"],
    internal_signatures: &[],
    related_formats: &[],
};
