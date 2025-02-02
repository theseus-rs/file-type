use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205727: FileFormat = FileFormat {
    id: 28_205_727,
    source_type: SourceType::Wikidata,
    name: "AVS X image",
    extensions: &["avs", "mbfavs", "x"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
