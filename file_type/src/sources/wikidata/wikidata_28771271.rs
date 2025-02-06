use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771271: FileFormat = FileFormat {
    id: 28_771_271,
    source_type: SourceType::Wikidata,
    name: "MSA (Magic Shadow Archiver)",
    extensions: &["msa"],
    media_types: &["application/vnd.msa-disk-image"],
    signatures: &[],
    related_formats: &[],
};
