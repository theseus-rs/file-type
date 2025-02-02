use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117843186: FileFormat = FileFormat {
    id: 117_843_186,
    source_type: SourceType::Wikidata,
    name: "Calculus EZ-Fax file",
    extensions: &["ezf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
