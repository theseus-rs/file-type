use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111167729: FileFormat = FileFormat {
    id: 111_167_729,
    source_type: SourceType::Wikidata,
    name: "ACD/HNMR Calculated Spectrum file",
    extensions: &["hsp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
