use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126087526: FileFormat = FileFormat {
    id: 126_087_526,
    source_type: SourceType::Wikidata,
    name: "Husqvarna / Premier+ Embroidery Stitch File",
    extensions: &["vp4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
