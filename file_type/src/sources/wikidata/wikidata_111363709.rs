use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363709: FileFormat = FileFormat {
    id: 111_363_709,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif XF 'voices' format",
    extensions: &["x3v"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
