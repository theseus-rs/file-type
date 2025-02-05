use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126951711: FileFormat = FileFormat {
    id: 126_951_711,
    source_type: SourceType::Wikidata,
    name: "NetRexx source code file",
    extensions: &["nrx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
