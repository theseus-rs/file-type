use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_16996920: FileFormat = FileFormat {
    id: 16_996_920,
    source_type: SourceType::Wikidata,
    name: "Windows Setup Information File",
    extensions: &["inf"],
    media_types: &[
        "application/inf",
        "application/x-setupscript",
        "application/x-wine-extension-inf",
        "text/x-inf",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
