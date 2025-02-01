use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966930: FileFormat = FileFormat {
    id: 27_966_930,
    puid: "wikidata/27966930",
    name: "QSF",
    extensions: &["miniqsf", "qsf", "qsflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
