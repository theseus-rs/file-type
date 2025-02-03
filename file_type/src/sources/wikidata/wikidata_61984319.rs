use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61984319: FileFormat = FileFormat {
    id: 61_984_319,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visual FoxPro Class Library",
    extensions: &["vcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
