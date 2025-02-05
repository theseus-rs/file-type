use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66686595: FileFormat = FileFormat {
    id: 66_686_595,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Template file format",
    extensions: &["wpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
