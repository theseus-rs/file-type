use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128597273: FileFormat = FileFormat {
    id: 128_597_273,
    source_type: SourceType::Wikidata,
    name: "Ampl file",
    extensions: &["run"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
