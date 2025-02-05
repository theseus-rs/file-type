use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51717333: FileFormat = FileFormat {
    id: 51_717_333,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint Presentation, version 95",
    extensions: &["ppt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
