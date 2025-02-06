use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206318: FileFormat = FileFormat {
    id: 28_206_318,
    source_type: SourceType::Wikidata,
    name: "Img Software Set Colormapped Image",
    extensions: &["p"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
