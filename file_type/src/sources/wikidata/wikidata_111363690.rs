use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363690: FileFormat = FileFormat {
    id: 111_363_690,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif XS 'waves' format",
    extensions: &["x0w"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
