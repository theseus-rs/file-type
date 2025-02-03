use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51370102: FileFormat = FileFormat {
    id: 51_370_102,
    source_type: SourceType::Wikidata,
    name: "Microsoft Print File",
    extensions: &["prn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
