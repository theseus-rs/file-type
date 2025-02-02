use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51333820: FileFormat = FileFormat {
    id: 51_333_820,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint Presentation Show",
    extensions: &["pps"],
    media_types: &["application/vnd.ms-powerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
