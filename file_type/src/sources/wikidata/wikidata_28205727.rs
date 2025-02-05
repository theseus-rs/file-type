use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205727: FileFormat = FileFormat {
    id: 28_205_727,
    source_type: SourceType::Wikidata,
    name: "AVS X image",
    extensions: &["avs", "mbfavs", "x"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
