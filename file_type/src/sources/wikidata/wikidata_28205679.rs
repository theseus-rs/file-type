use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205679: FileFormat = FileFormat {
    id: 28_205_679,
    source_type: SourceType::Wikidata,
    name: "Amber ARR Bitmap Image",
    extensions: &["arr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
