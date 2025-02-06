use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850050: FileFormat = FileFormat {
    id: 105_850_050,
    source_type: SourceType::Wikidata,
    name: "Cabal info (with rem)",
    extensions: &["cabal"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
