use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120984438: FileFormat = FileFormat {
    id: 120_984_438,
    source_type: SourceType::Wikidata,
    name: "Microsoft Publisher Template",
    extensions: &["pub"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
