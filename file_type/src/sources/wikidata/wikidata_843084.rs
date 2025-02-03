use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_843084: FileFormat = FileFormat {
    id: 843_084,
    source_type: SourceType::Wikidata,
    name: "Microsoft Document Imaging Format",
    extensions: &["mdi"],
    media_types: &["image/vnd.ms-modi"],
    internal_signatures: &[],
    related_formats: &[],
};
