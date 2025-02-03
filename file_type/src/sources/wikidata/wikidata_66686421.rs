use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66686421: FileFormat = FileFormat {
    id: 66_686_421,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Word Processor 1-3 for DOS and 2 for Windows",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
