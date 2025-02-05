use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47487619: FileFormat = FileFormat {
    id: 47_487_619,
    source_type: SourceType::Wikidata,
    name: "GEM Metafile",
    extensions: &["gem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
