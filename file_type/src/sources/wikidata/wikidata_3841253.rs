use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3841253: FileFormat = FileFormat {
    id: 3_841_253,
    source_type: SourceType::Wikidata,
    name: "MDL Molfile",
    extensions: &["mol"],
    media_types: &["chemical/x-mdl-molfile"],
    signatures: &[],
    related_formats: &[],
};
