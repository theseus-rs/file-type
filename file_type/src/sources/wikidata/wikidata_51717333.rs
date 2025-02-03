use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51717333: FileFormat = FileFormat {
    id: 51_717_333,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint Presentation, version 95",
    extensions: &["ppt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
