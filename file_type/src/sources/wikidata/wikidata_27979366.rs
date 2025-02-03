use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979366: FileFormat = FileFormat {
    id: 27_979_366,
    source_type: SourceType::Wikidata,
    name: "Flash Media Manifest",
    extensions: &["f4m"],
    media_types: &["application/f4m"],
    internal_signatures: &[],
    related_formats: &[],
};
