use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51800009: FileFormat = FileFormat {
    id: 51_800_009,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Macro, version 4",
    extensions: &["xla", "xlm"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[],
    related_formats: &[],
};
