use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131232034: FileFormat = FileFormat {
    id: 131_232_034,
    source_type: SourceType::Wikidata,
    name: "Allotrope Data Format",
    extensions: &["adf"],
    media_types: &["application/x-hdf5"],
    signatures: &[],
    related_formats: &[],
};
