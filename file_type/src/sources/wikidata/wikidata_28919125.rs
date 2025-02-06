use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919125: FileFormat = FileFormat {
    id: 28_919_125,
    source_type: SourceType::Wikidata,
    name: "Final Cut Pro X project",
    extensions: &["fcpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
