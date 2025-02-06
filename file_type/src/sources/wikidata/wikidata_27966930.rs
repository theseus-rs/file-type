use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966930: FileFormat = FileFormat {
    id: 27_966_930,
    source_type: SourceType::Wikidata,
    name: "QSF",
    extensions: &["miniqsf", "qsf", "qsflib"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
