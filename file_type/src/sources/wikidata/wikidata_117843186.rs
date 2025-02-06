use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117843186: FileFormat = FileFormat {
    id: 117_843_186,
    source_type: SourceType::Wikidata,
    name: "Calculus EZ-Fax file",
    extensions: &["ezf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
