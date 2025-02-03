use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2804859: FileFormat = FileFormat {
    id: 2_804_859,
    source_type: SourceType::Wikidata,
    name: "VDA-FS",
    extensions: &["vda", "vdafs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
