use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119574681: FileFormat = FileFormat {
    id: 119_574_681,
    source_type: SourceType::Wikidata,
    name: "Kid Pix File",
    extensions: &["kpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
