use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117843578: FileFormat = FileFormat {
    id: 117_843_578,
    source_type: SourceType::Wikidata,
    name: "Faxable TIF",
    extensions: &["ftf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
