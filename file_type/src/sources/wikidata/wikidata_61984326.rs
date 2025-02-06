use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61984326: FileFormat = FileFormat {
    id: 61_984_326,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visual FoxPro Project",
    extensions: &["pjx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
