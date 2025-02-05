use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126179164: FileFormat = FileFormat {
    id: 126_179_164,
    source_type: SourceType::Wikidata,
    name: "Macintosh PICT Image 2.0",
    extensions: &["pct", "pic", "pict"],
    media_types: &["image/x-pict"],
    signatures: &[],
    related_formats: &[],
};
