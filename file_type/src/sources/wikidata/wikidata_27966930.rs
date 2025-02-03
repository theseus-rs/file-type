use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966930: FileFormat = FileFormat {
    id: 27_966_930,
    source_type: SourceType::Wikidata,
    name: "QSF",
    extensions: &["miniqsf", "qsf", "qsflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
