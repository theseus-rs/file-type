use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205653: FileFormat = FileFormat {
    id: 28_205_653,
    source_type: SourceType::Wikidata,
    name: "Abekas YUV",
    extensions: &["yuv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
