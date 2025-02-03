use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111363709: FileFormat = FileFormat {
    id: 111_363_709,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif XF 'voices' format",
    extensions: &["x3v"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
