use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61984326: FileFormat = FileFormat {
    id: 61_984_326,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visual FoxPro Project",
    extensions: &["pjx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
