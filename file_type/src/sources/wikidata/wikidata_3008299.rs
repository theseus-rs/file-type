use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3008299: FileFormat = FileFormat {
    id: 3_008_299,
    source_type: SourceType::Wikidata,
    name: "xorg.conf",
    extensions: &["xorg.conf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
