use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979154: FileFormat = FileFormat {
    id: 27_979_154,
    source_type: SourceType::Wikidata,
    name: "ArtWorx Data Format",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
