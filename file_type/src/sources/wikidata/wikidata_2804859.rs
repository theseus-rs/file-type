use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2804859: FileFormat = FileFormat {
    id: 2_804_859,
    source_type: SourceType::Wikidata,
    name: "VDA-FS",
    extensions: &["vda", "vdafs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
