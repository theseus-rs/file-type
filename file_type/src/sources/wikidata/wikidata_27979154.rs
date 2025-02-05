use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979154: FileFormat = FileFormat {
    id: 27_979_154,
    source_type: SourceType::Wikidata,
    name: "ArtWorx Data Format",
    extensions: &["adf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
