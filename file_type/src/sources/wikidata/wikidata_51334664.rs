use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51334664: FileFormat = FileFormat {
    id: 51_334_664,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint Presentation, version 4",
    extensions: &["ppt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
