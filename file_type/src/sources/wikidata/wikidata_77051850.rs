use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_77051850: FileFormat = FileFormat {
    id: 77_051_850,
    source_type: SourceType::Wikidata,
    name: "Cal3D Xml Animation File",
    extensions: &["xaf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
