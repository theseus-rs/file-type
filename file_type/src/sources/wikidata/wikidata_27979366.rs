use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979366: FileFormat = FileFormat {
    id: 27_979_366,
    source_type: SourceType::Wikidata,
    name: "Flash Media Manifest",
    extensions: &["f4m"],
    media_types: &["application/f4m"],
    signatures: &[],
    related_formats: &[],
};
