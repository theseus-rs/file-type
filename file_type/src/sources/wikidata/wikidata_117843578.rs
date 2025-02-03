use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117843578: FileFormat = FileFormat {
    id: 117_843_578,
    source_type: SourceType::Wikidata,
    name: "Faxable TIF",
    extensions: &["ftf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
